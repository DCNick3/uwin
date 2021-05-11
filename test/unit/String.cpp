//
// Created by dcnick3 on 4/26/21.
//

#include "gtest/gtest.h"

#include "str/uconv.h"

#pragma clang diagnostic push
#pragma ide diagnostic ignored "cert-err58-cpp"

using namespace uwin;

TEST(String, UConv) {
    str::wide wide_1(u"[JH科学 × TeddyLoid] ElectricAlice");
    str::narrow narrow_2("Lorem superpos\xe9s valise pourparlers r\xeaver chiots rendez-vous naissance Eiffel myrtille. "
                       "Gr\xe8ves Arc de Triomphe encore pourquoi sentiments baguette p\xe9""diluve une projet sentiments "
                       "saperlipopette vachement le. Brume \xe9ph\xe9m\xe8re baguette Bordeaux en fait sommet avoir minitel.\n\n"
                       "    Nous avoir parole la nous moussant. Superpos\xe9s tatillon exprimer voler St Emilion ressemblant "
                       "\xe9ph\xe9m\xe8re bourguignon. Bourguignon penser c\xe2lin mill\xe9sime peripherique annoncer enfants "
                       "enfants vachement nuit formidable encombr\xe9 \xe9panoui chiots. Arc truc cacato\xe8s lorem fl\xe2ner.");
    auto native_1 = str::wide_to_native(wide_1);
    auto wide_2 = str::narrow_to_wide(1252, narrow_2);
    auto native_2 = str::narrow_to_native(1252, narrow_2);

    auto native_wide_2 = str::wide_to_native(wide_2);
    auto narrow_wide_2 = str::wide_to_narrow(1252, wide_2);
    auto narrow_native_2 = str::native_to_narrow(1252, native_2);

    ASSERT_EQ(native_1.raw_str(), "[JH科学 × TeddyLoid] ElectricAlice");
    // for some reason ASSERT_EQ does not work here
    ASSERT_TRUE(wide_2 == u"Lorem superposés valise pourparlers rêver chiots rendez-vous naissance Eiffel myrtille. Grèves Arc de Triomphe encore pourquoi sentiments baguette pédiluve une projet sentiments saperlipopette vachement le. Brume éphémère baguette Bordeaux en fait sommet avoir minitel.\n\n    Nous avoir parole la nous moussant. Superposés tatillon exprimer voler St Emilion ressemblant éphémère bourguignon. Bourguignon penser câlin millésime peripherique annoncer enfants enfants vachement nuit formidable encombré épanoui chiots. Arc truc cacatoès lorem flâner.");
    ASSERT_EQ(native_2.raw_str(), "Lorem superposés valise pourparlers rêver chiots rendez-vous naissance Eiffel myrtille. Grèves Arc de Triomphe encore pourquoi sentiments baguette pédiluve une projet sentiments saperlipopette vachement le. Brume éphémère baguette Bordeaux en fait sommet avoir minitel.\n\n    Nous avoir parole la nous moussant. Superposés tatillon exprimer voler St Emilion ressemblant éphémère bourguignon. Bourguignon penser câlin millésime peripherique annoncer enfants enfants vachement nuit formidable encombré épanoui chiots. Arc truc cacatoès lorem flâner.");

    ASSERT_EQ(native_wide_2.raw_str(), native_2.raw_str());
    ASSERT_EQ(narrow_wide_2.raw_str(), narrow_2.raw_str());
    ASSERT_EQ(narrow_native_2.raw_str(), narrow_2.raw_str());

}
